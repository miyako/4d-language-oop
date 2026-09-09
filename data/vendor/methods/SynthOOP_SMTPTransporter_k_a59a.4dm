// property read
var $receiver : 4D.SMTPTransporter
$receiver:=4D.SMTPTransporter.new(New object("host"; "smtp.example.com"))
var $read1 : Boolean
$read1:=$receiver.keepAlive
// property write
$receiver:=4D.SMTPTransporter.new(New object("host"; "smtp.example.com"))
$receiver.keepAlive:=True
