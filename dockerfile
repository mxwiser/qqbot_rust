FROM docker.1ms.run/alpine:latest
RUN sed -i 's#https\?://dl-cdn.alpinelinux.org/alpine#https://mirrors.tuna.tsinghua.edu.cn/alpine#g' /etc/apk/repositories
RUN apk update && apk add tzdata 
ENV TZ=Asia/Shanghai
# RUN cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime 
# RUN echo "Asia/Shanghai" > /etc/timezone
VOLUME /app/assets
WORKDIR /app
COPY ./target/x86_64-unknown-linux-musl/release/qqbot_rust ./qqbot_rust
COPY ./bot.env ./bot.env
# 暴露应用运行的端口（根据需要修改）
EXPOSE 8080

# 设置容器启动时的默认命令
CMD ["./qqbot"]