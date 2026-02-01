<?php
header("Cache-Control: no-cache");
header("Content-Type: text/html");

$query_string = $_SERVER['QUERY_STRING'] ?? '';

print "<!DOCTYPE html>";
print "<html><head><title>GET Request Echo</title>";
print "</head><body><h1 align='center'>GET Request Echo</h1>";
print "<hr>";
print "<p><b>Query String:</b> $query_string</p>";

if (!empty($_GET)) {
    foreach ($_GET as $key => $value) {
        print "$key = $value<br/>\n";
    }
}

print "</body></html>";
?>
