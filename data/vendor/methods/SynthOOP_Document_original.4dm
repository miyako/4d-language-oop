// property read [4D.File]
// Document is abstract; synthesized against its concrete subclass File.
var $receiver : 4D.File
$receiver:=File("/PACKAGE/README.md")
var $read1 : 4D.File
$read1:=$receiver.original
// property read [4D.Folder]
// Document is abstract; synthesized against its concrete subclass File.
$receiver:=File("/PACKAGE/README.md")
var $read2 : 4D.Folder
$read2:=$receiver.original
