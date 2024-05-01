use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

const RUN_PLAYER: &str = "./assets/run.png";
const IDLE_PLAYER: &str = "./assets/idle.png";
const JUMP_PLAYER: &str = "./assets/jump.png";
const BLOCK: &str = "./assets/block.png";

struct Obstacle {
    position: Vector2,
    size: Vector2,
}

struct PlayerP {
    position: Vector2,
    size: Vector2,
}

struct Player {
    position: Vector2,
    speed: f32,
    sprite: Texture2D,
    frame_x: f32,
    frame_y: f32
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Teste")
        .vsync()
        .build();


    let mut player =  Player{
        position: Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0),
        speed: 2.0,
        sprite: rl.load_texture(&thread, IDLE_PLAYER).unwrap(),
        frame_x: 0.0,
        frame_y: 0.0,
    }; 
    let mut elapsed_time = 0.0;
    let background_image = rl.load_texture(&thread, "./assets/fundo.png").unwrap();
    let mut sprite_block = rl.load_texture(&thread, BLOCK).unwrap();

    let mut map: Vec<i32> = vec![
        0,1,2,2,2,2,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,4,5,5,5,5,6,0,0,0,0,0,1,2,2,2,2,2,2,2,3,
        0,0,0,0,0,0,0,0,0,0,0,0,4,5,5,5,5,5,5,5,6,

    ];

    let mut player_mirror = 32.0;



    while !rl.window_should_close() {
        // Update Lógica do jogo aqui...
        let frame_time = rl.get_frame_time();

        let mut player_rec = Rectangle::new(player.frame_x, player.frame_y, player_mirror, 32.0);
        elapsed_time += frame_time;

        



        if rl.is_key_down(KEY_A){
            player.position.x -= player.speed;
            player.sprite = rl.load_texture(&thread, RUN_PLAYER).unwrap();
            player_mirror = -32.0;
        }

        else if rl.is_key_down(KEY_D){
            player.position.x += player.speed;
            player_mirror = 32.0;
            player.sprite = rl.load_texture(&thread, RUN_PLAYER).unwrap();
    
        }
        else if rl.is_key_down(KEY_S){player.position.y += player.speed;}
        
        else{
            player.sprite = rl.load_texture(&thread, IDLE_PLAYER).unwrap();
        }

         if rl.is_key_down(KEY_W){
            //player.position.y -= player.speed;
            player.sprite = rl.load_texture(&thread, JUMP_PLAYER).unwrap();
            player.position.y -= player.speed;
        }

        if elapsed_time >= 0.05{
            elapsed_time = 0.0;
            player.frame_x += 32.0;
        }




        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLUE);


        let mut block_rec: Rectangle = Rectangle::default();
        let mut colisao = player_rec.check_collision_recs(&block_rec);
        println!("{}", colisao);



        d.draw_fps(10, 10);

        //
        // Draw
        //
        //

        let mut block_x: f32 = 0.0;
        let mut block_y: f32 = 0.0;
        let mut width: f32 = 0.0;
        let mut height: f32 = 256.0;

        let mut n_block: i32 = 0;

        let mut sprite_selec_y: f32 = 0.0;
        let mut sprite_selec_x: f32 = 0.0;


        for mut block in &map{

            if *block != 0{
                match block{
                    1 => {sprite_selec_x = 96.0;}
                    2 => {sprite_selec_x = 112.0;}
                    3 => {sprite_selec_x = 128.0}
                    4 => {sprite_selec_x = 96.0;sprite_selec_y = 32.0}
                    5 => {sprite_selec_x = 112.0; sprite_selec_y = 32.0}
                    6 => {sprite_selec_x = 128.0; sprite_selec_y = 32.0}
                    _ => {}
                }

                block_rec = Rectangle::new(sprite_selec_x, sprite_selec_y, 16.0, 16.0);

                d.draw_texture_rec(
                    &sprite_block,
                    block_rec,
                    Vector2::new(width, height),
                    Color::WHITE);

            };
            n_block += 1;
            if n_block <= 20{
                width += 16.0;
            }else{
                width = 0.0;
                height += 16.0;
                n_block = 0;
            }

            sprite_selec_x = 0.0;
            sprite_selec_y = 0.0;


        }



        d.draw_texture_rec(
            &player.sprite,
            player_rec,
            Vector2::new(player.position.x, player.position.y),
            Color::WHITE);

//        d.draw_texture_rec(
//            &image,
//            Rectangle::new(0.0, 0.0, 92.0, 92.0),
//            Vector2::new(200.0, 70.0),
//            Color::WHITE
//            )

      };
}

