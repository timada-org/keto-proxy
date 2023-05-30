check:
	curl -G --silent \
     --data-urlencode "subject_id=john" \
     --data-urlencode "relation=decypher" \
     --data-urlencode "namespace=messages" \
     --data-urlencode "object=02y_15_4w350m3" \
     https://keto.timada.localhost/relation-tuples/check
