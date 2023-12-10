#!/usr/bin/perl
use warnings;
use List::Util qw(reduce);

# Take differences between consecutive numbers in a list
sub diff {
    my ($numbers_ref,) = @_;
    my @numbers = @{$numbers_ref};
    my @diffs = ();
    for (my $i = 0; $i < scalar(@numbers) - 1; $i++) {
        push(@diffs, $numbers[$i + 1] - $numbers[$i]);
    }
    return @diffs;
}

# Recursively take differences until all numbers are 0, then extrapolate the top end value
sub take_differences {
    my ($numbers_ref, $current_value) = @_;
    my @numbers = @{$numbers_ref};

    if (scalar(grep { $_ == 0 } @numbers) == scalar(@numbers)) {
        return $current_value;
    }
    my @new_numbers = diff(\@numbers);
    return take_differences(\@new_numbers, $current_value + $numbers[scalar(@numbers) - 1]);
}

# Recursively take differences until all numbers are 0, then extrapolate the top start value
sub take_differences_start {
    my ($numbers_ref, $starts_ref) = @_;
    my @numbers = @{$numbers_ref};
    my @starts = @{$starts_ref};
    push(@starts, $numbers[0]);

    if (scalar(grep { $_ == 0 } @numbers) == scalar(@numbers)) {
        my $line_extrapolated_value = 0;
        my @reverse_starts = reverse(@starts);
        for my $start (@reverse_starts) {
            my $next_extrapolated_value = $start - $line_extrapolated_value;
            $line_extrapolated_value = $next_extrapolated_value;
        }
        return $line_extrapolated_value;
    }

    my @new_numbers = diff(\@numbers);
    return take_differences_start(\@new_numbers, \@starts);
}

# Get the extrapolated value for the end of the line
sub get_end_extrapolated_value {
    my ($row,) = @_;
    chomp $row;
    my @numbers = map { int($_) } split(' ', $row);
    return take_differences(\@numbers, 0);
}

#
sub get_start_extrapolated_value {
    my ($row,) = @_;
    chomp $row;
    my @numbers = map { int($_) } split(' ', $row);
    my @starts = ();
    return take_differences_start(\@numbers, \@starts);
}

# Solve the puzzle, extrapolating a value for each line and summing them
sub solve {
    my ($filename, $extrapolate_func) = @_;
    open(my $fh, '<', $filename) or die "Could not open file '$filename' $!";
    my @rows = ();
    while (my $row = <$fh>) {
        push(@rows, $row);
    }
    close $fh;

    my @extrapolated_values = map $extrapolate_func -> ($_), @rows;
    my $answer = reduce { $a + $b } @extrapolated_values;
    print("$answer\n");    
}

solve('puzzle_9/example.txt', \&get_end_extrapolated_value);
solve('puzzle_9/input.txt', \&get_end_extrapolated_value);
solve('puzzle_9/example.txt', \&get_start_extrapolated_value);
solve('puzzle_9/input.txt', \&get_start_extrapolated_value);
