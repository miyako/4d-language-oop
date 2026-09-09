// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $formula1 : 4D.Function
$formula1:=Formula(1+2)
var $result2 : Boolean
$result2:=$receiver.some(1; $formula1; "synthText")
// overload 1
$receiver:=New collection(1; 2; 3)
var $result3 : Boolean
$result3:=$receiver.some(1; "synthText"; "synthText")
