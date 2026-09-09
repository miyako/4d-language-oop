// overload 0
// Document is abstract; synthesized against its concrete subclass File.
var $receiver : 4D.File
$receiver:=File("/PACKAGE/README.md")
var $result1 : Text
$result1:=$receiver.getText("synthText"; 1)
// overload 0 [optional:omit-charSetName]
// Document is abstract; synthesized against its concrete subclass File.
$receiver:=File("/PACKAGE/README.md")
var $result2 : Text
$result2:=$receiver.getText()
// overload 0 [optional:omit-breakMode]
// Document is abstract; synthesized against its concrete subclass File.
$receiver:=File("/PACKAGE/README.md")
var $result3 : Text
$result3:=$receiver.getText("synthText")
// overload 1
// Document is abstract; synthesized against its concrete subclass File.
$receiver:=File("/PACKAGE/README.md")
var $result4 : Text
$result4:=$receiver.getText(1; 1)
// overload 1 [optional:omit-charSetNum]
// Document is abstract; synthesized against its concrete subclass File.
$receiver:=File("/PACKAGE/README.md")
var $result5 : Text
$result5:=$receiver.getText()
// overload 1 [optional:omit-breakMode]
// Document is abstract; synthesized against its concrete subclass File.
$receiver:=File("/PACKAGE/README.md")
var $result6 : Text
$result6:=$receiver.getText(1)
