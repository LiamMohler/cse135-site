<?php
print "Cache-Control: no-cache\n";
print "Content-Type: text/html\n\n";

$date = date('Y-m-d H:i:s');
$ip = $_SERVER['REMOTE_ADDR'];

print "<!DOCTYPE html>";
print "<html>";
print "<head>";
print "<title>Hello PHP</title>";
print "</head>";
print "<body>";
print "<h1>Hello, PHP!</h1>";
print "<p>Greeting from Team Liam</p>";
print "<p>This page was generated with the PHP programming language</p>";
print "<p>Generated at: $date</p>";
print "<p>Your IP Address: $ip</p>";
print "</body>";
print "</html>";
?>