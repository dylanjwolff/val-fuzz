
while [ 1 ]
do
        echo $(date) $(ps -L -u $1 | cut -d ' ' -f 1,2 | sort | uniq | wc -l);
    sleep 4
done
