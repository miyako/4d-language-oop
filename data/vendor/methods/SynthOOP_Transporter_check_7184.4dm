// overload 0
// Transporter is abstract; synthesized against its concrete subclass SMTPTransporter.
var $receiver : 4D.SMTPTransporter
$receiver:=4D.SMTPTransporter.new(New object("host"; "smtp.example.com"))
var $result1 : Object
$result1:=$receiver.checkConnection()
