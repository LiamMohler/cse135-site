<?php
header("Cache-Control: no-cache");
header("Content-Type: text/html");

print "<!DOCTYPE html>";
print "<html>";
print "<head>";
print "<title>PHP Environment</title>";
print "</head>";
print "<body>";
print "<h1>PHP Environment Variables</h1>";
print "<table border='1'>";
print "<tr><th>Variable</th><th>Value</th></tr>";

foreach ($_SERVER as $key => $value) {
    print "<tr><td>$key</td><td>$value</td></tr>";
}

print "</table>";
print "</body>";
print "</html>";
?>