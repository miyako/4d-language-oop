// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $result1 : Boolean
$result1:=$receiver.equal(New collection; ck diacritical)
// overload 0 [optional:omit-option]
$receiver:=New collection(1; 2; 3)
var $result2 : Boolean
$result2:=$receiver.equal(New collection)
