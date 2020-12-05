use Switch;
use Data::Dumper;

my $input = "../input/2020/day5.txt";

open my $file, $input;
my $max;
my %seen;
while (<$file>) {
	chomp;
	my $row_max = 127;
	my $row_min = 0;
	my $seat_max = 7;
	my $seat_min = 0;
	for my $half (split(//)) {
		switch ($half) {
			case 'F' { $row_max = int(($row_max + $row_min) / 2) }
			case 'B' { $row_min = int(($row_max + $row_min) / 2) }
			case 'L' { $seat_max = int(($seat_max + $seat_min) / 2) }
			case 'R' { $seat_min = int(($seat_max + $seat_min) / 2) }
		}
	}
	my $id = $row_max * 8 + $seat_max;
	$seen{$id} = ();
	if ($id > $max) {
		$max = $id;
	}
}
print "Part 1: $max\n";
for (my $i=$max;$i>0;$i--) {
	if (!exists $seen{$i}) {
		print "Part 2: $i\n";
		last;
	}
}
