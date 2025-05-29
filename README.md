This is an unfineshed proof of concept for a robotic system for putting styrofoam insulation on buildings. 

After cloning code, you can start backend with cargo run within web-api directory, and you can start frontend with npm start within drawing directory. After that you can access app in web browser at http://localhost:8090.

I am currently thinking about changing rendering engine from robotics-agnostic babylon.js 3d engine to Gazebo, but it has not been done so far. 

Application serves the purpose of visualization of the process of how robots would put the insulation from begining to the end (it is still unfinished). 

Wireframe preview
![{6A755FFA-56BA-4251-BB8C-8E3950CA11CA}](https://github.com/user-attachments/assets/6e5c9bf3-0f88-4555-863c-30295e6e91fb)
Finished state with fully shown insulation
![{A0390B8F-029B-4361-AFC0-C8F25FFBFD39}](https://github.com/user-attachments/assets/c8578371-8447-4612-8f41-c680666f08bc)
Animation mode shows insulation being set in progress
![{DADA02B0-89E5-44A2-8075-B9FE78EFE45B}](https://github.com/user-attachments/assets/d119beb9-a52f-4a15-8aad-a48a3c6a1a2e)

