#/usr/bin/perl

use warnings;
use strict;

my %deps = ();

print "[package]\n";
print "name = \"aoc2022\"\n";
print "version = \"0.1.0\"\n";
print "edition = \"2021\"\n";
print "\n";

foreach (`ls`) {
    chomp;
    if (/^[0-9][0-9]$/) {
        my $number = $_;
        my $name = `ls $number`;
        chomp $name;

        # print "$number: $name\n";

        print "[[bin]]\n";
        print "name = \"$name\"\n";
        print "path = \"$number/$name/src/main.rs\"\n";
        print "\n";

        my $tomlpath = "$number/$name/Cargo.toml";
        open(TOML, '<', $tomlpath) or die $!;

        my $in_deps = 0;
        while (<TOML>) {
            chomp;
            if ($in_deps) {
                $deps{$_} = 1;
            }

            if (/^\[dependencies\]$/) {
                $in_deps = 1;
            }
        }
    }
}

print "[dependencies]\n";
my @deps = keys(%deps);
@deps = sort @deps;
foreach my $dep (@deps) {
    print "$dep\n";
}
print "\n";

