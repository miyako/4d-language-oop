// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $result1 : Collection
$result1:=$receiver.query("synthText")
// overload 1
$receiver:=New collection(1; 2; 3)
var $result2 : Collection
$result2:=$receiver.query("synthText"; "synthText")
// overload 2
$receiver:=New collection(1; 2; 3)
var $result3 : Collection
$result3:=$receiver.query("synthText"; New object)
