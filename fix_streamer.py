with open('src/pages/streamer.rs', 'r') as f:
    content = f.read()
if content.count('{') > content.count('}'):
    content += '\n}\n'
with open('src/pages/streamer.rs', 'w') as f:
    f.write(content)
