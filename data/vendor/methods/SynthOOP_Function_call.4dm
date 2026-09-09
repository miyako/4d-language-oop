// overload 0
// Function is abstract; synthesized against its concrete subclass Formula.
var $receiver : 4D.Function
$receiver:=Formula(1+2)
var $result1 : Variant
$result1:=$receiver.call()
// overload 1
// Function is abstract; synthesized against its concrete subclass Formula.
$receiver:=Formula(1+2)
var $result2 : Variant
$result2:=$receiver.call(New object; "synthText")
