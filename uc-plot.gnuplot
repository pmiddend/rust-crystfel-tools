set boxwidth 1
set yrange[0:1200]
set style fill solid 1.0
set terminal png size 800,600
set output outputfile
plot inputfile lc rgb inputcolor with boxes title title
