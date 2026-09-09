// property read
// Document is abstract; synthesized against its concrete subclass File.
var $receiver : 4D.File
$receiver:=File("/PACKAGE/README.md")
var $read1 : Boolean
$read1:=$receiver.isFolder
