#!/usr/bin/env perl

use strict;
use warnings FATAL => 'all';
use Time::HiRes qw/gettimeofday tv_interval/;

my $start = [gettimeofday];

my %m;

for (my $i = 0; $i < 10000; ++$i) {
	$m{$i} = $i;
}

my $ans = 0;
foreach my $k (keys %m) {
	$ans += $m{$k};
}

my $elapsed = tv_interval($start, [gettimeofday]);

print "answer=$ans, $elapsed sec\n";
