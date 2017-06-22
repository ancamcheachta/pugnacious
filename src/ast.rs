/**
    /pugnacious/src/ast.rs
*/

// Node enums and variants
pub enum Mode {
    append,
    prepend,
    replace,
}
pub enum Node {
    /** Note: the commented node variants are from interfaces documented under
        _Abstract Node Types_:
        https://github.com/pugjs/pug-ast-spec/blob/master/parser.md.
        
        Since they're considered abstract in the documentation, I realised after
        writing the code that these variants are unlikely ever to be used 
        directly during compilation.  For that reason, I thought the abstract
        node types from the documentation to be better candidates for traits,
        which you'll find later.  However, I'll leave the commented variants
        below until it's clear that they won't be needed, then they can be 
        removed.
    */
    /*AttributedNode {
        attr: Vec<Attribute>,
        attribute_blocks: Vec<String>,
    },*/
    Block {
        nodes: Vec<Node>,
    },
    BlockComment {
        block: Box<Node>,
        buffer: bool,
        val: String,
    },
    /*BlockNode {
        block: Option<Box<Node>>,
    },*/
    Case {
        block: Option<Box<Node>>, // Must be Option<Node::WhenBlock>
        expr: Box<Node>,
    },
    Code {
        block: Option<Box<Node>>,
        buffer: bool,
        is_inline: bool,
        must_escape: bool,
        val: String,
    },
    Comment {
        buffer: bool,
        val: String,
    },
    CommonComment {
        buffer: bool,
        val: String,
    },
    CommonTag {
        attr: Vec<Attribute>,
        attribute_blocks: Vec<String>,
        block: Box<Node>,
        is_inline: bool,
        self_closing: String,
        val: String,
    },
    Conditional {
        alternate: Option<Box<Node>>,
        consequent: Box<Node>,
        test: String,
    },
    Doctype {
        val: String,
    },
    Each {
        alternate: Option<Box<Node>>,
        block: Option<Box<Node>>,
        key: Box<Node>,
        obj: Box<Node>,
        val: Box<Node>,
    },
    /*ExpressionNode {
        expr: Box<Node>,
    },*/
    Extends {
        file: Box<Node>,
    },
    FileNode {
        file: Box<Node>,
    },
    FileReference {
        path: String
    },
    Filter {
        attrs: Vec<Attribute>,
        block: Option<Box<Node>>,
        name: String,
    },
    FilterNode {
        attrs: Vec<Attribute>,
        name: String,
    },
    Include {
        block: Option<Box<Node>>,
        file: Box<Node>,
    },
    IncludeFilter {
        attrs: Vec<Attribute>,
        name: String,
    },
    InterpolatedTag {
        attr: Vec<Attribute>,
        attribute_blocks: Vec<String>,
        block: Box<Node>,
        expr: Box<Node>,
        is_inline: bool,
        self_closing: String,
        val: String,
    },
    JavascriptExpression(String),
    JavscriptIdentifier(String),
    Mixin {
        args: String,
        attr: Vec<Attribute>,
        attribute_blocks: Vec<String>,
        nodes: Vec<Node>,
        call: bool,
        name: Box<Node>,
    },
    MixinBlock,
    NamedBlock {
        mode: Mode,
        name: String,
        nodes: Vec<Node>
    },
    //PlaceholderNode,
    RawInclude {
        file: Box<Node>,
        filters: Vec<Node>,
    },
    Tag {
        attr: Vec<Attribute>,
        attribute_blocks: Vec<String>,
        block: Box<Node>,
        is_inline: bool,
        name: String,
        self_closing: String,
        val: String,
    },
    Text {
        val: String,
    },
    /*ValueNode {
        val: String,
    },*/
    When {
        block: Option<Box<Node>>,
        expr : Box<Node>,
    },
    WhenBlock {
        nodes: Vec<Node>,   // Must be Vec<Node::When>
    },
    While {
        block: Option<Box<Node>>,
        test: Box<Node>,
    },
    YieldBlock,
    None,

}

// Abstract node traits
pub trait NodeCommon {
    fn get_column() -> Option<i32>;
    fn get_filename() -> Option<String>;
    fn get_line() -> i32;
}

pub trait AttributedNode: NodeCommon {
    fn get_attr() -> Vec<Attribute>;
    fn get_attribute_blocks() -> Vec<Node>;
}

pub trait Block: NodeCommon {
    fn get_nodes() -> Vec<Node>;
}

pub trait BlockNode: NodeCommon {
    fn get_block() -> Option<Box<Node>>;
}

pub trait CommonComment: ValueNode {
    fn get_buffer() -> bool;
}

pub trait CommonTag: AttributedNode + BlockNode {
    fn get_is_inline() -> bool;
    fn get_self_closing() -> bool;
}

pub trait ExpressionNode: NodeCommon {
    fn get_expression() -> Box<Node>;
}

pub trait FileNode: NodeCommon {
    fn get_file() -> Box<Node>;
}

pub trait FilterNode: NodeCommon {
    fn get_attrs() -> Vec<Attribute>;
    fn get_name() -> String;
}

pub trait PlaceholderNode: NodeCommon {}

pub trait ValueNode: NodeCommon {
    fn get_val() -> String;
}

// Node structs
pub struct Attribute {
    must_escape: bool,
    name: String,
    val: String,
}

pub struct NodeData {
    pub column: Option<i32>,
    pub filename: Option<String>,
    pub line: i32,
}