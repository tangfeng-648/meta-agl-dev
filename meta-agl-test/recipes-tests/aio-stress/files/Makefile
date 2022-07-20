CFLAGS = -Wall
TARGET = aio-stress


all: $(TARGET)


$(TARGET): aio-stress.c
	$(CC) $(CFLAGS) $^ -o $@ -lpthread -laio


clean:
	$(RM) $(TARGET)
