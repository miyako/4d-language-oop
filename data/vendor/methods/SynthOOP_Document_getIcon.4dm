// overload 0
// Document is abstract; synthesized against its concrete subclass File.
var $receiver : 4D.File
$receiver:=File("/PACKAGE/README.md")
var $result1 : Picture
$result1:=$receiver.getIcon(1)
// overload 0 [optional:omit-size]
// Document is abstract; synthesized against its concrete subclass File.
$receiver:=File("/PACKAGE/README.md")
var $result2 : Picture
$result2:=$receiver.getIcon()
