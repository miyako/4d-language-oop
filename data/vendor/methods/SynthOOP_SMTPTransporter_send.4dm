// overload 0
var $receiver : 4D.SMTPTransporter
$receiver:=4D.SMTPTransporter.new(New object("host"; "smtp.example.com"))
var $result1 : Object
$result1:=$receiver.send(New object)
