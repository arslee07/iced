use crate::{Renderer, Settings};

use iced_graphics::Viewport;
use iced_native::{futures, mouse};
use raw_window_handle::HasRawWindowHandle;

/// A window graphics backend for iced powered by `wgpu`.
#[derive(Debug)]
pub struct Compositor {
    device: wgpu::Device,
    queue: wgpu::Queue,
    format: wgpu::TextureFormat,
}

impl iced_graphics::window::Compositor for Compositor {
    type Settings = Settings;
    type Renderer = Renderer;
    type Surface = wgpu::Surface;
    type SwapChain = wgpu::SwapChain;

    fn new(settings: Self::Settings) -> Self {
        let (device, queue) = futures::executor::block_on(async {
            let adapter = wgpu::Adapter::request(
                &wgpu::RequestAdapterOptions {
                    power_preference: if settings.antialiasing.is_none() {
                        wgpu::PowerPreference::Default
                    } else {
                        wgpu::PowerPreference::HighPerformance
                    },
                    compatible_surface: None,
                },
                wgpu::BackendBit::PRIMARY,
            )
            .await
            .expect("Request adapter");

            adapter
                .request_device(&wgpu::DeviceDescriptor {
                    extensions: wgpu::Extensions {
                        anisotropic_filtering: false,
                    },
                    limits: wgpu::Limits { max_bind_groups: 2 },
                })
                .await
        });

        Self {
            device,
            queue,
            format: settings.format,
        }
    }

    fn create_renderer(&mut self, settings: Settings) -> Renderer {
        Renderer::new(crate::Backend::new(&mut self.device, settings))
    }

    fn create_surface<W: HasRawWindowHandle>(
        &mut self,
        window: &W,
    ) -> wgpu::Surface {
        wgpu::Surface::create(window)
    }

    fn create_swap_chain(
        &mut self,
        surface: &Self::Surface,
        width: u32,
        height: u32,
    ) -> Self::SwapChain {
        self.device.create_swap_chain(
            surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: self.format,
                width,
                height,
                present_mode: wgpu::PresentMode::Mailbox,
            },
        )
    }

    fn draw<T: AsRef<str>>(
        &mut self,
        renderer: &mut Self::Renderer,
        swap_chain: &mut Self::SwapChain,
        viewport: &Viewport,
        output: &<Self::Renderer as iced_native::Renderer>::Output,
        overlay: &[T],
    ) -> mouse::Interaction {
        let frame = swap_chain.get_next_texture().expect("Next frame");

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None },
        );

        let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
            }],
            depth_stencil_attachment: None,
        });

        let mouse_interaction = renderer.backend_mut().draw(
            &mut self.device,
            &mut encoder,
            &frame.view,
            viewport,
            output,
            overlay,
        );

        self.queue.submit(&[encoder.finish()]);

        mouse_interaction
    }
}
