// First: ᵃ
// Second: ᵇ
use candle_core::{Result};
use candle_nn::Module;

pub struct ConvolutionPipeline{
    pub convᵃ: candle_nn::Conv1d,
    pub convᵇ: candle_nn::Conv1d,
    pub layer_normᵃ: candle_nn::LayerNorm,
    pub layer_normᵇ: candle_nn::LayerNorm,
}


impl ConvolutionPipeline{
    pub fn new(
        channels: usize,
        kernel_size: usize,
        vs: candle_nn::VarBuilder
    ) -> Result<Self>{
        let conv_cfgᵃ = candle_nn::Conv1dConfig{
            padding: kernel_size/2,
            stride: 1,
            dilation: 1,
            groups: 1,
            cudnn_fwd_algo: None
        };
        let conv_cfgᵇ = candle_nn::Conv1dConfig{
            padding: kernel_size/2,
            stride: 1,
            dilation: 1,
            groups: 1,
            cudnn_fwd_algo: None
        };
        
        let convᵃ = candle_nn::conv1d(channels, channels, kernel_size, conv_cfgᵃ, vs.pp("convᵃ"))?;
        let convᵇ = candle_nn::conv1d(channels, channels, kernel_size, conv_cfgᵇ, vs.pp("convᵇ"))?;

        let layer_normᵃ = candle_nn::layer_norm(channels, 1e-5, vs.pp("layer_normᵃ"))?;
        let layer_normᵇ = candle_nn::layer_norm(channels, 1e-5, vs.pp("layer_normᵇ"))?;
        Ok(Self{convᵃ, convᵇ, layer_normᵃ, layer_normᵇ})
    }
}

impl Module for ConvolutionPipeline{
    fn forward(&self, xs: &candle_core::Tensor) -> Result<candle_core::Tensor> {
        let mut hidden = self.convᵃ.forward(&xs)?;
        hidden = hidden.transpose(1, 2)?;
        hidden = self.layer_normᵃ.forward(&hidden)?;
        hidden = hidden.transpose(1, 2)?;
        hidden = hidden.relu()?;

        hidden = self.convᵇ.forward(&hidden)?;
        hidden = hidden.transpose(1, 2)?;
        hidden = self.layer_normᵇ.forward(&hidden)?;
        hidden = hidden.transpose(1, 2)?;
        (hidden + xs)?.relu()
    }
}