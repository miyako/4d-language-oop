// overload 0
var $receiver : 4D.FileHandle
$receiver:=File("/PACKAGE/data.txt").open("write")
var $result1 : Text
$result1:=$receiver.readText("synthText")
// overload 0 [optional:omit-stopChar]
$receiver:=File("/PACKAGE/data.txt").open("write")
var $result2 : Text
$result2:=$receiver.readText()
