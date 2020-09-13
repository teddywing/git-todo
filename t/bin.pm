package Bin;

use strict;
use warnings;

use Exporter qw(import);
our @EXPORT = qw($BIN);

use File::Spec;

our $BIN = File::Spec->rel2abs('git-todo');

1;
