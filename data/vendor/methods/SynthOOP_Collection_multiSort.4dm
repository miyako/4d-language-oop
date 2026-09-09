// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $result1 : Collection
$result1:=$receiver.multiSort()
// overload 1
$receiver:=New collection(1; 2; 3)
var $result2 : Collection
$result2:=$receiver.multiSort(New collection)
// overload 2
$receiver:=New collection(1; 2; 3)
var $formula3 : 4D.Function
$formula3:=Formula(1+2)
var $result4 : Collection
$result4:=$receiver.multiSort($formula3; New collection)
