use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};

use crate::window::window::{
    DISTANCE_PROJ_PLANE, MINIMAP_SCALING, NUM_COLS, NUM_RAYS, NUM_ROWS, TILE_SIZE, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

use super::app::App;

impl<'a> App<'a> {
    fn generate_3d_projection(&mut self) {
        for x in 0..NUM_RAYS {
            if let Some(ray) = self.game.rays.get(x as usize) {
                let distance = ray.distance;
                let ray_angle = ray.angle;
                let player_angle = self.player.rotation_angle;
                let perp_dist = distance * ((ray_angle - player_angle).cos());
                let proj_wall_height =
                    (TILE_SIZE as f64 / perp_dist) * (*DISTANCE_PROJ_PLANE as f64);
                let wall_segment_height = proj_wall_height as i32;

                let mut top_wall_pixel: i32 =
                    (WINDOW_HEIGHT as i32 / 2) - (wall_segment_height / 2);
                if top_wall_pixel < 0 {
                    top_wall_pixel = 0;
                }

                let mut bottom_wall_pixel: i32 =
                    (WINDOW_HEIGHT as i32 / 2) + (wall_segment_height / 2);
                if bottom_wall_pixel > WINDOW_HEIGHT as i32 {
                    bottom_wall_pixel = WINDOW_HEIGHT as i32;
                }

                // ceiling
                for y in 0..top_wall_pixel {
                    let index = (WINDOW_WIDTH * y as u32 + x) as usize;
                    self.color_buffer.buffer[index] = 0xFF444444;
                }

                // wall
                for y in top_wall_pixel..bottom_wall_pixel {
                    let index = (WINDOW_WIDTH * y as u32 + x) as usize;
                    if self.game.rays[x as usize].is_vertical_collision {
                        self.color_buffer.buffer[index] = 0xFFFFFFFF;
                    } else {
                        self.color_buffer.buffer[index] = 0xFFCCCCCC;
                    }
                }

                // floor
                for y in bottom_wall_pixel..WINDOW_HEIGHT as i32 {
                    let index = (WINDOW_WIDTH * y as u32 + x) as usize;
                    self.color_buffer.buffer[index] = 0xFF777777;
                }
            }
        }
    }

    fn render_color_buffer(&mut self) -> Result<(), String> {
        let color_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                self.color_buffer.buffer.as_ptr() as *const u8,
                self.color_buffer.buffer.len() * 4,
            )
        };

        self.color_buffer
            .texture
            .update(None, color_bytes, WINDOW_WIDTH as usize * 4)
            .map_err(|err| format!("Error updating texture: {:?}", err))?;

        self.canvas
            .copy(&self.color_buffer.texture, None, None)
            .map_err(|err| format!("Error copying texture to canvas: {:?}", err))?;

        Ok(())
    }

    fn render_map(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        self.canvas.fill_rect(Rect::new(
            0,
            0,
            (MINIMAP_SCALING * NUM_COLS as f64 * TILE_SIZE as f64) as u32,
            (MINIMAP_SCALING * NUM_ROWS as f64 * TILE_SIZE as f64) as u32,
        ))?;

        for i in 0..NUM_ROWS {
            for j in 0..NUM_COLS {
                let x_tile = j * TILE_SIZE;
                let y_tile = i * TILE_SIZE;

                let tile_color = if self.game.game_map[i as usize][j as usize] != 0 {
                    255
                } else {
                    0
                };

                self.canvas
                    .set_draw_color(Color::RGBA(tile_color, tile_color, tile_color, 255));
                let map_tile = Rect::new(
                    (x_tile as f64 * MINIMAP_SCALING) as i32,
                    (y_tile as f64 * MINIMAP_SCALING) as i32,
                    (TILE_SIZE as f64 * MINIMAP_SCALING) as u32,
                    (TILE_SIZE as f64 * MINIMAP_SCALING) as u32,
                );
                self.canvas
                    .fill_rect(map_tile)
                    .map_err(|err| format!("Error filling rect {:?}", err))?;
            }
        }
        Ok(())
    }

    fn render_rays(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));

        for i in 0..NUM_RAYS {
            let ray_start = Point::new(
                (MINIMAP_SCALING * self.player.x) as i32,
                (MINIMAP_SCALING * self.player.y) as i32,
            );
            let ray_end = Point::new(
                (MINIMAP_SCALING * self.game.rays[i as usize].x_collision) as i32,
                (MINIMAP_SCALING * self.game.rays[i as usize].y_collision) as i32,
            );
            self.canvas.draw_line(ray_start, ray_end)?;
        }

        Ok(())
    }

    fn render_player(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));

        let player_rect = Rect::new(
            (self.player.x * MINIMAP_SCALING) as i32,
            (self.player.y * MINIMAP_SCALING) as i32,
            (self.player.width * MINIMAP_SCALING) as u32,
            (self.player.height * MINIMAP_SCALING) as u32,
        );
        self.canvas.fill_rect(player_rect)?;

        let length = 30.0;
        let line_end_x = (MINIMAP_SCALING * self.player.x) as i32
            + (length * self.player.rotation_angle.cos()) as i32;
        let line_end_y = (MINIMAP_SCALING * self.player.y) as i32
            + (length * self.player.rotation_angle.sin()) as i32;

        let start_point = Point::new(
            (MINIMAP_SCALING * self.player.x) as i32,
            (MINIMAP_SCALING * self.player.y) as i32,
        );
        let end_point = Point::new(line_end_x, line_end_y);
        self.canvas.draw_line(start_point, end_point)?;

        Ok(())
    }

    pub fn render(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        self.canvas.clear();

        self.generate_3d_projection();

        self.render_color_buffer()?;
        self.color_buffer.clear(Color::RGBA(0, 0, 0, 255))?;

        self.render_map()?;
        self.render_rays()?;
        self.render_player()?;
        self.canvas.present();

        Ok(())
    }
}
