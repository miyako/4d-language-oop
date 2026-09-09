// overload 0
var $receiver : 4D.POP3Transporter
$receiver:=4D.POP3Transporter.new(New object("host"; "pop.example.com"))
var $result1 : Object
$result1:=$receiver.getMail(1; True)
// overload 0 [optional:omit-headerOnly]
$receiver:=4D.POP3Transporter.new(New object("host"; "pop.example.com"))
var $result2 : Object
$result2:=$receiver.getMail(1)
