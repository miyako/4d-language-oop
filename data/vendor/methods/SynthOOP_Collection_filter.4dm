// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $formula1 : 4D.Function
$formula1:=Formula(1+2)
var $result2 : Collection
$result2:=$receiver.filter($formula1; "synthText")
// overload 1
$receiver:=New collection(1; 2; 3)
var $result3 : Collection
$result3:=$receiver.filter("synthText"; "synthText")
