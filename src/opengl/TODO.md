TODO:
- Choose game object out of list, and be able to modify it's MVP (including scale/roatation/transform) matrices - dynamically
    - Make this menu appear and disappear with special key binding
    - Isn't view matrix static for all game objects? Projection aswell?
      - Either way, maybe we should calculate (projection * view * model) on CPU bc this calculation only changes every once in a while
          - Verify this online
- Read from mesh data (figure out how to create 3D shapes, how to export, common filetypes & how to read them)