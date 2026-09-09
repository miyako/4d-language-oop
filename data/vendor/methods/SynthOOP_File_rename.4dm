// overload 0
var $receiver : 4D.File
$receiver:=File("/PACKAGE/README.md")
var $result1 : 4D.File
$result1:=$receiver.rename("synthText")
