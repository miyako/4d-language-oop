// overload 0
var $receiver : 4D.Session
$receiver:=Session
var $result1 : Text
$result1:=$receiver.createOTP(1)
// overload 0 [optional:omit-lifespan]
$receiver:=Session
var $result2 : Text
$result2:=$receiver.createOTP()
