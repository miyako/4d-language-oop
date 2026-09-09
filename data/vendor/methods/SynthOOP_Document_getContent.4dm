// overload 0
// Document is abstract; synthesized against its concrete subclass File.
var $receiver : 4D.File
$receiver:=File("/PACKAGE/README.md")
var $result1 : 4D.Blob
$result1:=$receiver.getContent()
