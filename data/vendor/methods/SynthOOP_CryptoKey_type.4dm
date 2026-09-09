// property read
var $receiver : 4D.CryptoKey
$receiver:=4D.CryptoKey.new(New object("type"; "RSA"; "size"; 2048))
var $read1 : Text
$read1:=$receiver.type
// property write
$receiver:=4D.CryptoKey.new(New object("type"; "RSA"; "size"; 2048))
$receiver.type:="synthText"
