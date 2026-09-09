// property read
// Transporter is abstract; synthesized against its concrete subclass SMTPTransporter.
var $receiver : 4D.SMTPTransporter
$receiver:=4D.SMTPTransporter.new(New object("host"; "smtp.example.com"))
var $read1 : Integer
$read1:=$receiver.port
// property write
// Transporter is abstract; synthesized against its concrete subclass SMTPTransporter.
$receiver:=4D.SMTPTransporter.new(New object("host"; "smtp.example.com"))
$receiver.port:=1
