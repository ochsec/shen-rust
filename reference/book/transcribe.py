import os
import logging
import base64
import openai

# Initialize OpenAI API key
openai.api_key = os.getenv("OPENAI_API_KEY")

class GPTImageTranscriber:
    def __init__(self, image_dir, output_file):
        self.image_dir = image_dir
        self.output_file = output_file
        logging.basicConfig(level=logging.INFO)
        self.logger = logging.getLogger(__name__)

    def _encode_image(self, image_path):
        """
        Encode image to base64 for GPT-4o processing.
        """
        with open(image_path, "rb") as image_file:
            return base64.b64encode(image_file.read()).decode('utf-8')

    def transcribe_image(self, image_path):
        """
        Use GPT-4o model to transcribe text from an image with advanced parsing.
        """
        try:
            base64_image = self._encode_image(image_path)
            
            response = openai.ChatCompletion.create(
                model="gpt-4o",
                messages=[
                    {
                        "role": "user", 
                        "content": [
                            {
                                "type": "text", 
                                "text": """Carefully transcribe this image. 
                                Follow these guidelines:
                                - For mathematical formulas, use LaTeX notation enclosed in $ for inline or $$ for block formulas
                                - For code snippets, use markdown code blocks with appropriate language specifiers
                                - Preserve original formatting and structure
                                - Be precise and detailed"""
                            },
                            {
                                "type": "image_url",
                                "image_url": {
                                    "url": f"data:image/jpeg;base64,{base64_image}"
                                }
                            }
                        ]
                    }
                ],
                max_tokens=4096
            )
            
            return response.choices[0].message.content
        
        except Exception as e:
            self.logger.error(f"Error transcribing {image_path}: {e}")
            return ""

    def transcribe_images(self):
        """
        Transcribe text from images and save output in Markdown format.
        """
        image_extensions = [".png", ".jpg", ".jpeg", ".tiff", ".bmp", ".gif"]
        image_files = [
            os.path.join(self.image_dir, f)
            for f in os.listdir(self.image_dir)
            if os.path.splitext(f)[1].lower() in image_extensions
        ]
        image_files.sort()

        markdown_content = []
        for image_path in image_files:
            self.logger.info(f"Processing {image_path}")
            transcription = self.transcribe_image(image_path)

            if transcription:
                # Add Markdown headers and content
                markdown_content.append(f"## {os.path.basename(image_path)}\n\n")
                markdown_content.append(transcription)
                markdown_content.append("\n\n---\n\n")  # Add a separator between images

        # Write transcriptions to Markdown file
        with open(self.output_file, "w", encoding="utf-8") as f:
            f.writelines(markdown_content)

        self.logger.info(
            f"Transcription complete. Results written to {self.output_file}"
        )

def main():
    transcriber = GPTImageTranscriber(
        image_dir="./images", output_file="./transcriptions.md"
    )
    transcriber.transcribe_images()

if __name__ == "__main__":
    main()
