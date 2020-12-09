use Switch;
use Data::Dumper;
use strict;
use warnings;

my $input = "../input/2020/day6.txt";

open my ($file), $input;
my $total;
my %answered;
while (<$file>) {
	chomp;
	if (/\S/) {
		map { $answered{$_} = () } split(//);
	} else {
		$total += scalar(keys %answered);
		%answered = ();
	}
}
$total += scalar(keys %answered);
print "Part 1: $total\n";

open $file, $input;
$total = 0;
%answered = map { $_ => 1 } split(//, 'abcdefghijklmnopqrstuvwxyz');
while (<$file>) {
	chomp;
	if (/\S/) {
		my %current = map { $_ => 1 } split(//);
		my @delete = grep { !exists $current{$_}} keys %answered;
		for my $k (@delete) {
				delete $answered{$k};
		}
	} else {
		$total += scalar(keys %answered);
		%answered = map { $_ => 1 } split(//, 'abcdefghijklmnopqrstuvwxyz');
	}
}
$total += scalar(keys %answered);
print "Part 2: $total\n";
