// overload 0
var $receiver : 4D.FileHandle
$receiver:=File("/PACKAGE/data.txt").open("write")
var $result1 : Text
$result1:=$receiver.readLine()
