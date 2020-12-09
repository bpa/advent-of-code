use Switch;
use Data::Dumper;

open my $file, "../input/2020/day4.txt";
my @required = qw/byr iyr eyr hgt hcl ecl pid/;
my %existing;
my $count;
while (<$file>) {
	if (/\S/) {
		for my $f (split /\s+/) {
			my ($name) = split(':', $f);
			$existing{$name} = ();
		}
	} else {
		$count++ unless grep { !exists $existing{$_} } @required;
		%existing = ()
	}
}
$count++ unless grep { !exists $existing{$_} } @required;
print "Part 1: $count\n";


open my $file, "../input/2020/day4.txt";
%existing = ();
$count = 0;
while (<$file>) {
	chomp;
	if (/\S/) {
		for my $f (split /\s+/) {
			my ($name, $value) = split(':', $f);
			switch($name) {
				case "byr" { $existing{$name} = $value if $value >= 1920 && $value <= 2002 }
				case "iyr" { $existing{$name} = $value if $value >= 2010 && $value <= 2020 }
				case "eyr" { $existing{$name} = $value if $value >= 2020 && $value <= 2030 }
				case "hgt" { 
					my ($h, $type) = /(\d+)(cm|in)/;
					if ($type eq 'in') {
						$existing{$name} = $value if $h >= 59 && $h <= 76;
					} elsif ($type eq 'cm') {
						$existing{$name} = $value if $h >= 150 && $h <= 193;
					}
				}
				case "hcl" { $existing{$name} = $value if $value =~ /^#[a-z0-9]{6}$/ }
				case "ecl" { $existing{$name} = $value if $value =~ /^amb|blu|brn|gry|grn|hzl|oth$/ }
				case "pid" { $existing{$name} = $value if $value =~ /^\d{9}$/ }
			}
		}
	} else {
		$count++ unless grep { !exists $existing{$_} } @required;
		%existing = ();
	}
}
$count++ unless grep { !exists $existing{$_} } @required;
%existing = ();
print "Part 2: $count\n";
