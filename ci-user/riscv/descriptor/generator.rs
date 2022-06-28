use std::fmt::*;

macro_rules! as_str_polyfill {
    ($x: expr, $r: expr) => {{
        let mut y = $x.clone();
        if let Some(x) = y.next() {
            $r.split_at(x.as_ptr() as usize - $r.as_ptr() as usize).1
        } else {
            ""
        }
    }};
}
#[derive(Debug, Clone)]
struct EnumerationDescriptor<'a> {
    enumerations: Vec<(&'a str, usize)>,
}
impl<'a> EnumerationDescriptor<'a> {
    pub fn parse(enums: &'a str) -> Self {
        let mut counter = 0;
        let list = enums.split(";");
        let mut e = Vec::new();
        for tup in list {
            let mut t = tup.split("=");
            let n = t.next().unwrap();
            if let Some(new_id) = t.next() {
                counter = new_id.parse().unwrap();
            }
            e.push((n, counter));
            counter += 1;
        }
        EnumerationDescriptor { enumerations: e }
    }
    fn generate_enum(&self, name: &str) -> String {
        let mut ret = String::new();
        write!(
            &mut ret,
            "#[derive(Copy, Clone, Debug)]
#[repr(usize)]
"
        )
        .unwrap();
        write!(&mut ret, "pub enum {}{{\n", name).unwrap();
        let mut branches = String::new();
        for e in self.enumerations.iter() {
            write!(&mut ret, "    {} = {},\n", e.0, e.1).unwrap();
            write!(&mut branches, "            {} => Self::{},\n", e.1, e.0).unwrap();
        }

        write!(
            &mut ret,
            "}}
impl {}{{
    fn from(x: usize)->Self{{
        match x{{
{}            _ => unreachable!()
        }}
    }}
}}
",
            name, branches
        )
        .unwrap();
        return ret;
    }
}
#[derive(Debug, Clone)]
struct BitFieldDescriptor<'a> {
    name: &'a str,
    description: &'a str,
    lo: usize,
    hi: usize,
    ed: Option<(&'a str, EnumerationDescriptor<'a>)>,
}

impl<'a> BitFieldDescriptor<'a> {
    pub fn parse(desc: &'a str) -> Self {
        let mut parts = desc.split(",");
        let name = parts.next().unwrap();
        let hi = parts.next().unwrap().parse::<usize>().unwrap();
        let lo = parts.next().unwrap().parse::<usize>().unwrap();
        let (lo, hi) = if lo < hi { (lo, hi) } else { (hi, lo) };
        let use_enum = parts.next().unwrap();
        let ed = if use_enum != "number" {
            let opts = parts.next().unwrap();
            Some((use_enum, EnumerationDescriptor::parse(opts)))
        } else {
            None
        };
        let description = as_str_polyfill!(parts, desc);
        BitFieldDescriptor {
            name,
            lo,
            hi,
            description,
            ed,
        }
    }
    pub fn generate_enum(&self) -> Option<String> {
        if let Some((n, e)) = &self.ed {
            Some(e.generate_enum(n))
        } else {
            None
        }
    }
    pub fn flag_type(&self) -> &str {
        if let Some((n, _)) = self.ed {
            n
        } else {
            if self.lo == self.hi {
                "bool"
            } else {
                "usize"
            }
        }
    }
    fn mask(&self) -> String {
        format!("{}", (1usize << (self.hi - self.lo + 1)) - 1)
    }
    fn getter(&self) -> String {
        if self.lo == self.hi {
            return format!("self.bits.get_bit({})", self.lo);
        } else if self.flag_type() != "usize" {
            return format!(
                "{}::from(self.bits.get_bits({}..{}))",
                self.flag_type(),
                self.lo,
                self.hi + 1
            );
        } else {
            return format!("self.bits.get_bits({}..{})", self.lo, self.hi + 1);
        }
    }
    fn setter(&self) -> String {
        if self.lo == self.hi {
            return format!("self.bits.set_bit({}, val);", self.lo);
        } else if self.flag_type() != "usize" {
            return format!(
                "self.bits.set_bits({}..{}, val as usize);",
                self.lo,
                self.hi + 1
            );
        } else {
            return format!("self.bits.set_bits({}..{}, val);", self.lo, self.hi + 1);
        }
    }
    fn generate_read_write(&self) -> String {
        format!(
            "    /// {}
    #[inline] 
    pub fn {}(&self)->{}{{
        {}
    }}
    #[inline]
    pub fn set_{}(&mut self, val: {}){{
        {}
    }}\n",
            self.description,
            self.name,
            self.flag_type(),
            self.getter(),
            self.name,
            self.flag_type(),
            self.setter()
        )
    }

    fn generate_bit_set(&self) -> String {
        format!(
            "    pub fn set_{}()->bool{{
        unsafe {{csr::csrrc({}) & {} !=0}}
    }}
    pub fn clear_{}()->bool{{
        unsafe {{csr::csrrs({}) & {} !=0 }}
    }}\n",
            self.name,
            1usize << self.lo,
            1usize << self.lo,
            self.name,
            1usize << self.lo,
            1usize << self.lo
        )
    }
    fn generate_bitops(&self) -> String {
        format!(
            "    set_clear_csr!(
    ///{}
    , set_{}, clear_{}, 1 << {});\n",
            self.description, self.name, self.name, self.lo
        )
    }
}

#[derive(Debug, Clone)]
struct CSRDescriptor<'a> {
    name: &'a str,
    id: usize,
    description: &'a str,
    bfs: Vec<BitFieldDescriptor<'a>>,
}

impl<'a> CSRDescriptor<'a> {
    fn canonical_name(&self) -> String {
        self.name.to_lowercase()
    }
    pub fn parse(d: &'a str) -> Self {
        let mut parts = d.split("\n");
        let name = parts.next().unwrap();
        let id = parts.next().unwrap().parse::<usize>().unwrap();
        let mut bfs = Vec::new();
        while let Some(x) = parts.next() {
            if x == "end" {
                break;
            } else {
                bfs.push(BitFieldDescriptor::parse(x));
            }
        }
        CSRDescriptor {
            name,
            id,
            description: as_str_polyfill!(parts, d),
            bfs,
        }
    }
    pub fn generate(&self) -> String {
        let mut trait_impls = String::new();
        let mut bit_sets = String::new();
        let mut enums = String::new();
        for bf in self.bfs.iter() {
            if bf.lo == bf.hi {
                write!(&mut bit_sets, "{}", bf.generate_bitops()).unwrap();
                //write!(&mut trait_impls, "{}",bf.generate_bit_set()).unwrap();
            }
            write!(&mut trait_impls, "{}", bf.generate_read_write()).unwrap();
            if let Some(x) = bf.generate_enum() {
                write!(&mut enums, "{}", x).unwrap();
            }
        }
        if &trait_impls == "" && &bit_sets == "" {
            format!(
                "
//! {}
read_csr_as_usize!({}, __read_{});
write_csr_as_usize!({}, __write_{});
",
                self.description,
                self.id,
                self.canonical_name(),
                self.id,
                self.canonical_name()
            )
        } else {
            format!(
                "
//! {}

use bit_field::BitField;

#[derive(Copy, Clone, Debug)]
pub struct {}{{\n    bits: usize,\n}}
impl {}{{
    #[inline]
    pub fn bits(&self) -> usize{{
        return self.bits;
    }}
    #[inline]
    pub fn from_bits(x: usize) -> Self{{
        return {}{{bits: x}};
    }}
    #[inline]
    pub unsafe fn write(&self){{
        _write(self.bits);
    }}
{}
}}
read_csr_as!({}, {}, __read_{});
write_csr!({}, __write_{});
set!({}, __set_{});
clear!({}, __clear_{});
// bit ops
{}
// enums
{}

",
                self.description,
                self.name,
                self.name,
                self.name,
                trait_impls,
                self.name,
                self.id,
                self.canonical_name(),
                self.id,
                self.canonical_name(),
                self.id,
                self.canonical_name(),
                self.id,
                self.canonical_name(),
                bit_sets,
                enums,
            )
        }
    }
}

fn main() {
    use std::io::Read;
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let csr = CSRDescriptor::parse(&buffer);
    println!("{}", csr.generate());
}
