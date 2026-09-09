// overload 0
// Function is abstract; synthesized against its concrete subclass Formula.
var $receiver : 4D.Function
$receiver:=Formula(1+2)
var $result1 : Variant
$result1:=$receiver.apply()
// overload 1
// Function is abstract; synthesized against its concrete subclass Formula.
$receiver:=Formula(1+2)
var $result2 : Variant
$result2:=$receiver.apply(New object; New collection)
// overload 1 [optional:omit-params]
// Function is abstract; synthesized against its concrete subclass Formula.
$receiver:=Formula(1+2)
var $result3 : Variant
$result3:=$receiver.apply(New object)
