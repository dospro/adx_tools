use std::collections::VecDeque;

use super::sample_decoder::Decoder;

pub struct AdxBufferCopy {
    pub(super) buffer: Vec<u8>,
    pub(super) cache: VecDeque<i16>,
    pub(super) buffer_offset: usize,
    pub(super) channels: u8,
    pub(super) has_loop: bool,
    pub(super) loop_start: u32,
    pub(super) loop_end: u32,
    pub(super) decoders: Vec<Decoder>,
}

impl AdxBufferCopy {
    pub fn get_next_sample(&mut self) -> Option<i16> {
        if self.cache.is_empty() {
            self.cache = match self.get_next_block() {
                Some(deque) => deque,
                None => return None,
            }
        }
        self.cache.pop_front()
    }

    fn get_next_block(&mut self) -> Option<VecDeque<i16>> {
        let mut channels_blocks: Vec<Vec<i16>> = Vec::new();
        if self.buffer_offset >= self.buffer.len() {
            return None;
        }

        for channel in 0..self.channels {
            // loop_end / 16 parece que queremos sacar en que bloque esta el loop_end
            // el *2 sera el numero de canales?
            // Por que le agregamos loop_end al restulado?
            let real_loop_end = self.loop_end as usize + 2 * (self.loop_end as usize / 16);
            if self.has_loop && self.buffer_offset >= real_loop_end {
                let real_loop_start = self.loop_start as usize + 2 * (self.loop_start as usize / 16);
                self.buffer_offset = real_loop_start;
            }

            let block: Vec<u8> = self.buffer[self.buffer_offset..self.buffer_offset + 18].to_vec();
            self.buffer_offset += 18;
            let decoded_block = self.decode_block(&block, channel);
            channels_blocks.push(decoded_block);
        }

        let decoded_block = self.join_channels_blocks(channels_blocks);
        Some(decoded_block)
    }


    fn decode_block(&mut self, block: &[u8], channel: u8) -> Vec<i16> {
        let scale = ((block[0] as u16) << 8) | block[1] as u16;
        block.into_iter()
            .skip(2)
            .flat_map(|sample| {
                let low_sample = sample & 0b1111;
                let high_sample = sample >> 4;
                let first = self.decoders[channel as usize].decode_sample(high_sample as i32, scale as i32) as i16;
                let second = self.decoders[channel as usize].decode_sample(low_sample as i32, scale as i32) as i16;
                vec![first, second]
            })
            .collect()
    }

    fn join_channels_blocks(&self, channel_blocks: Vec<Vec<i16>>) -> VecDeque<i16> {
        let mut deque: VecDeque<i16> = VecDeque::new();
        for i in 0..32 {
            for channel in 0..self.channels {
                deque.push_back(channel_blocks[channel as usize][i as usize]);
            }
        }
        deque
    }
}

impl Iterator for AdxBufferCopy {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_next_sample()
    }
}

pub struct AdxBuffer<'a> {
    pub(super) buffer: &'a [u8],
    pub(super) cache: VecDeque<i16>,
    pub(super) buffer_offset: usize,
    pub(super) channels: u8,
    pub(super) has_loop: bool,
    pub(super) loop_start: u32,
    pub(super) loop_end: u32,
    pub(super) decoders: Vec<Decoder>,

}

/**
* El buffer aun esta codificado.
* El buffer tiene el siguiente formato.
    Esta dividido en bloques de 18 bytes
    Los 2 primeros bytes son para la decodificacion, el resto son los samples.
    Cada byte contiene 2 samples de 4 bits, por lo tanto:
    Cada bloque tiene 32 muestras.
    Entonces, un bloque de 18 bytes producirá un bloque de audio de 32 bytes!

    Por otro lado, si tenemos mas de 1 canal, cada bloque representa un solo canal.
    Y cada bloque consecutivo va alternando entre los canales de manera ascendente.

* Sobre los loops

    El formato nos da en el header dos indices que nos dicen donde empieza y donde termina el loop
    de una rola.
    Estos indices son por muestra? Por bytes? O por bloque?

    Al parecer son muestras. Pero aqui los quiero convertir a bytes.



***/

impl<'a> AdxBuffer<'a> {
    pub fn get_next_sample(&mut self) -> Option<i16> {
        if self.cache.is_empty() {
            self.cache = match self.get_next_block() {
                Some(deque) => deque,
                None => return None,
            }
        }
        self.cache.pop_front()
    }

    fn get_next_block(&mut self) -> Option<VecDeque<i16>> {
        let mut channels_blocks: Vec<Vec<i16>> = Vec::new();
        if self.buffer_offset >= self.buffer.len() {
            return None;
        }

        for channel in 0..self.channels {
            // loop_end / 16 parece que queremos sacar en que bloque esta el loop_end
            // el *2 sera el numero de canales?
            // Por que le agregamos loop_end al restulado?
            let real_loop_end = self.loop_end as usize + 2 * (self.loop_end as usize / 16);
            if self.has_loop && self.buffer_offset >= real_loop_end {
                let real_loop_start = self.loop_start as usize + 2 * (self.loop_start as usize / 16);
                self.buffer_offset = real_loop_start;
            }

            let block = &self.buffer[self.buffer_offset..self.buffer_offset + 18];
            self.buffer_offset += 18;
            let decoded_block = self.decode_block(&block, channel);
            channels_blocks.push(decoded_block);
        }

        let decoded_block = self.join_channels_blocks(channels_blocks);
        Some(decoded_block)
    }


    fn decode_block(&mut self, block: &[u8], channel: u8) -> Vec<i16> {
        let scale = ((block[0] as u16) << 8) | block[1] as u16;
        block.into_iter()
            .skip(2)
            .flat_map(|sample| {
                let low_sample = sample & 0b1111;
                let high_sample = sample >> 4;
                let first = self.decoders[channel as usize].decode_sample(high_sample as i32, scale as i32) as i16;
                let second = self.decoders[channel as usize].decode_sample(low_sample as i32, scale as i32) as i16;
                vec![first, second]
            })
            .collect()
    }

    fn join_channels_blocks(&self, channel_blocks: Vec<Vec<i16>>) -> VecDeque<i16> {
        let mut deque: VecDeque<i16> = VecDeque::new();
        for i in 0..32 {
            for channel in 0..self.channels {
                deque.push_back(channel_blocks[channel as usize][i as usize]);
            }
        }
        deque
    }
}

impl<'a> Iterator for AdxBuffer<'a> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_next_sample()
    }
}