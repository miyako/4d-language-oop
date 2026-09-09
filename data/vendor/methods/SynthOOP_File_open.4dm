// overload 0
var $receiver : 4D.File
$receiver:=File("/PACKAGE/README.md")
var $result1 : 4D.FileHandle
$result1:=$receiver.open("synthText")
// overload 0 [optional:omit-mode]
$receiver:=File("/PACKAGE/README.md")
var $result2 : 4D.FileHandle
$result2:=$receiver.open()
// overload 1
$receiver:=File("/PACKAGE/README.md")
var $result3 : 4D.FileHandle
$result3:=$receiver.open(New object)
// overload 1 [optional:omit-options]
$receiver:=File("/PACKAGE/README.md")
var $result4 : 4D.FileHandle
$result4:=$receiver.open()
