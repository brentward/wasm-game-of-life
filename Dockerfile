FROM nginx

RUN rm /etc/nginx/conf.d/default.conf

COPY ./www/dist/ /usr/share/nginx/html
COPY ./nginx_conf/nginx.conf /etc/nginx/nginx.conf
COPY ./nginx_conf/mime.types /etc/nginx/mime.types
COPY ./nginx_conf/default.conf /etc/nginx/conf.d/default.conf

CMD sed -i -e 's/PORT_TOKEN/'"$PORT"'/g' /etc/nginx/conf.d/default.conf  && nginx -g 'daemon off;'
