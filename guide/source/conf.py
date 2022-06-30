# Configuration file for the Sphinx documentation builder.
#
# This file only contains a selection of the most common options. For a full
# list see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Path setup --------------------------------------------------------------

# If extensions (or modules to document with autodoc) are in another directory,
# add these directories to sys.path here. If the directory is relative to the
# documentation root, use os.path.abspath to make it absolute, like shown here.
#
# import os
# import sys
# sys.path.insert(0, os.path.abspath('.'))


# -- Project information -----------------------------------------------------

project = 'Open-Source-OS-Training-Camp-2022'
copyright = 'OS2022Summer'
author = 'Yifan Wu'
language = 'zh_CN'
html_search_language = 'zh'

# The full version, including alpha/beta/rc tags
# release = '0.1'


# -- General configuration ---------------------------------------------------

# Add any Sphinx extension module names here, as strings. They can be
# extensions coming with Sphinx (named 'sphinx.ext.*') or your custom
# ones.
extensions = [
    "sphinx_comments",
    "sphinx_tabs.tabs"
]

comments_config = {
   "utterances": {
       "repo": "LearningOS/rust-based-os-comp2022",
       "issue-term": "pathname",
       "label": "comments",
       "theme": "github-light",
       "crossorigin": "anonymous",
   }
}

# Add any paths that contain templates here, relative to this directory.
templates_path = ['_templates']

# List of patterns, relative to source directory, that match files and
# directories to ignore when looking for source files.
# This pattern also affects html_static_path and html_extra_path.
exclude_patterns = []


# -- Options for HTML output -------------------------------------------------

# The theme to use for HTML and HTML Help pages.  See the documentation for
# a list of builtin themes.
#
html_theme = 'furo'

# Add any paths that contain custom static files (such as style sheets) here,
# relative to this directory. They are copied after the builtin static files,
# so a file named "default.css" will overwrite the builtin "default.css".
html_static_path = ['_static']

html_css_files = [
    'my_style.css',
    #'dracula.css',
]

from pygments.lexer import RegexLexer
from pygments import token
from sphinx.highlighting import lexers

class RVLexer(RegexLexer):
    name = 'riscv'
    tokens = {
        'root': [
            # Comment
            (r'#.*\n', token.Comment),
            # General Registers
            (r'\b(?:x[1-2]?[0-9]|x30|x31|zero|ra|sp|gp|tp|fp|t[0-6]|s[0-9]|s1[0-1]|a[0-7]|pc)\b', token.Name.Attribute),
            # CSRs
            (r'\bs(?:status|tvec|ip|ie|counteren|scratch|epc|cause|tval|atp|)\b', token.Name.Constant),
            (r'\bm(?:isa|vendorid|archid|hardid|status|tvec|ideleg|ip|ie|counteren|scratch|epc|cause|tval)\b', token.Name.Constant),
            # Instructions
            (r'\b(?:(addi?w?)|(slti?u?)|(?:and|or|xor)i?|(?:sll|srl|sra)i?w?|lui|auipc|subw?|jal|jalr|beq|bne|bltu?|bgeu?|s[bhwd]|(l[bhw]u?)|ld)\b', token.Name.Decorator),
            (r'\b(?:csrr?[rws]i?)\b', token.Name.Decorator),
            (r'\b(?:ecall|ebreak|[msu]ret|wfi|sfence.vma)\b', token.Name.Decorator),
            (r'\b(?:nop|li|la|mv|not|neg|negw|sext.w|seqz|snez|sltz|sgtz|f(?:mv|abs|neg).(?:s|d)|b(?:eq|ne|le|ge|lt)z|bgt|ble|bgtu|bleu|j|jr|ret|call)\b', token.Name.Decorator),
            (r'(?:%hi|%lo|%pcrel_hi|%pcrel_lo|%tprel_(?:hi|lo|add))', token.Name.Decorator),
            # Directives
            (r'(?:.2byte|.4byte|.8byte|.quad|.half|.word|.dword|.byte|.dtpreldword|.dtprelword|.sleb128|.uleb128|.asciz|.string|.incbin|.zero)', token.Name.Function),
            (r'(?:.align|.balign|.p2align)', token.Name.Function),
            (r'(?:.globl|.local|.equ)', token.Name.Function),
            (r'(?:.text|.data|.rodata|.bss|.comm|.common|.section)', token.Name.Function),
            (r'(?:.option|.macro|.endm|.file|.ident|.size|.type)', token.Name.Function),
            (r'(?:.set|.rept|.endr|.macro|.endm|.altmacro)', token.Name.Function),
            # Number
            (r'\b(?:(?:0x|)[\da-f]+|(?:0o|)[0-7]+|\d+)\b', token.Number),
            # Labels
            (r'\S+:', token.Name.Builtin),
            # Whitespace
            (r'\s', token.Whitespace),
            # Other operators
            (r'[,\+\*\-\(\)\\%]', token.Text),
            # Hacks
            (r'(?:SAVE_GP|trap_handler|__switch|LOAD_GP|SAVE_SN|LOAD_SN|__alltraps|__restore)', token.Name.Builtin),
            (r'(?:.trampoline)', token.Name.Function),
            (r'(?:n)', token.Name.Entity),
            (r'(?:x)', token.Text),
        ],
    }

lexers['riscv'] = RVLexer()
