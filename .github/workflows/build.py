import os.path
current_directory = os.path.dirname(__file__)
parent_directory = os.path.split(os.path.split(current_directory)[0])[0]

if os.path.isfile(parent_directory+"/index.html"):
    os.remove(parent_directory+"/index.html")
    f=open(parent_directory+"/index.html","w")
    f.write('''
        <!DOCTYPE html>
        <html lang="en">
            <head> </head>
            <body style="margin: 0"></body>
        </html>

    ''')




f=open(os.path.join(parent_directory, "dist/index.html"),"r+")
writefile=f.read().replace("'/rust-website","'/Rust-Website/rust-website").replace('"/rust-website','"/Rust-Website/rust-website')
f.close()
f=open(os.path.join(parent_directory, "dist/index.html"),"w")
f.write(writefile)
f.close()

for filepath in os.listdir(parent_directory):
    if (os.path.isfile(os.path.join(parent_directory,filepath))) and filepath.endswith(".wasm") or filepath.endswith(".js"):
        os.remove(os.path.join(parent_directory,filepath))

for filepath in os.listdir(parent_directory+"/dist"):
    if os.path.isfile(os.path.join(parent_directory+"/dist",filepath)):
        print(os.path.join(parent_directory,filepath))
        if os.path.isfile(os.path.join(parent_directory,filepath)):
            os.remove(os.path.join(parent_directory,filepath))
        os.rename(os.path.join(parent_directory+"/dist",filepath),os.path.join(parent_directory,filepath))
