// overload 0
var $receiver : 4D.Session
$receiver:=Session
var $result1 : Boolean
$result1:=$receiver.setPrivileges("synthText")
// overload 1
$receiver:=Session
$receiver.setPrivileges(New collection)
// overload 2
$receiver:=Session
var $result2 : Boolean
$result2:=$receiver.setPrivileges(New object)
