// overload 0
var $receiver : 4D.DataStore
$receiver:=ds
var $result1 : Object
$result1:=$receiver.provideDataKey("synthText")
// overload 1
$receiver:=ds
var $result2 : Object
$result2:=$receiver.provideDataKey(New object)
