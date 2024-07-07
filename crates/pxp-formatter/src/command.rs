use pxp_bytestring::ByteString;

pub(crate) enum Command {
    /// A collection of commands.
    /// No real meaning to this – it's just a container and means of grouping commands.
    Commands(Vec<Command>),

    /// A command that is a simple set of characters to print.
    Text(ByteString),

    /// A group of commands.
    /// 
    /// This group should be printed as a single line if possible.
    /// When it's not possible, LineOrSpace should be inserted where applicable.
    Group(Vec<Command>),

    /// A command that forces a newline.
    HardLine,

    /// Inserts an empty line, without indentation.
    EmptyLine,

    /// Inserts a line when needed, does nothing when not needed.
    Line,

    /// Inserts a line when needed, or a space when not needed.
    LineOrSpace,

    /// Increases the indentation level.
    Indent,

    /// Decreases the indentation level.
    Unindent,
}