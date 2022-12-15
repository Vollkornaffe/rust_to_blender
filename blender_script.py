import bpy
import os
import errno
import stat

for handler in bpy.app.handlers.frame_change_pre:
    if handler.__name__ != 'update_geometry_from_pipe_input':
        continue
    print("removing", handler)
    bpy.app.handlers.frame_change_pre.remove(handler)

FIFO = 'blender_input'

print('Opening named pipe:', FIFO)
fifo = os.open(FIFO, os.O_RDONLY | os.O_NONBLOCK)

buffer = b''

def update_geometry_from_pipe_input(scene):
    try:
        buffer = os.read(fifo, 1024)
    except OSError as err:
        if err.errno == errno.EAGAIN or err.errno == errno.EWOULDBLOCK:
            buffer = None
        else:
            raise  # something else has happened -- better reraise
    
    n = len(buffer)
    
    
                
    if buffer:
        print("received something:", buffer, len(buffer))


bpy.app.handlers.frame_change_pre.append(update_geometry_from_pipe_input)