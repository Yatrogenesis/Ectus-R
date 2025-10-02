import React, { useCallback, useState } from 'react'
import { useDropzone } from 'react-dropzone'
import {
  CloudArrowUpIcon,
  DocumentTextIcon as DocumentIcon,
  PhotoIcon,
  FilmIcon,
  MusicalNoteIcon,
  ArchiveBoxIcon,
  XMarkIcon,
  CheckCircleIcon,
  ExclamationTriangleIcon,
} from '@heroicons/react/24/outline'
import { cn, formatBytes } from '@/lib/utils'

export interface FileWithPreview extends File {
  preview?: string
  id: string
  status: 'uploading' | 'success' | 'error'
  progress?: number
  error?: string
}

export interface FileUploadProps {
  onFilesChange: (files: FileWithPreview[]) => void
  accept?: Record<string, string[]>
  maxFiles?: number
  maxSize?: number
  multiple?: boolean
  disabled?: boolean
  className?: string
  children?: React.ReactNode
}

export const FileUpload: React.FC<FileUploadProps> = ({
  onFilesChange,
  accept,
  maxFiles = 10,
  maxSize = 10 * 1024 * 1024, // 10MB
  multiple = true,
  disabled = false,
  className,
  children,
}) => {
  const [files, setFiles] = useState<FileWithPreview[]>([])

  const onDrop = useCallback((acceptedFiles: File[], rejectedFiles: any[]) => {
    const newFiles: FileWithPreview[] = acceptedFiles.map(file => ({
      ...file,
      id: Math.random().toString(36).substr(2, 9),
      status: 'uploading' as const,
      preview: file.type.startsWith('image/') ? URL.createObjectURL(file) : undefined,
      progress: 0,
    }))

    const updatedFiles = multiple ? [...files, ...newFiles] : newFiles
    setFiles(updatedFiles)
    onFilesChange(updatedFiles)

    // Simulate upload progress
    newFiles.forEach(file => {
      simulateUpload(file)
    })

    // Handle rejected files
    if (rejectedFiles.length > 0) {
      console.warn('Rejected files:', rejectedFiles)
    }
  }, [files, multiple, onFilesChange])

  const simulateUpload = (file: FileWithPreview) => {
    let progress = 0
    const interval = setInterval(() => {
      progress += Math.random() * 30
      if (progress >= 100) {
        progress = 100
        clearInterval(interval)
        updateFileStatus(file.id, 'success', progress)
      } else {
        updateFileProgress(file.id, progress)
      }
    }, 200)
  }

  const updateFileProgress = (fileId: string, progress: number) => {
    setFiles(prev => prev.map(file =>
      file.id === fileId ? { ...file, progress } : file
    ))
  }

  const updateFileStatus = (fileId: string, status: FileWithPreview['status'], progress?: number, error?: string) => {
    setFiles(prev => {
      const updated = prev.map(file =>
        file.id === fileId ? { ...file, status, progress, error } : file
      )
      onFilesChange(updated)
      return updated
    })
  }

  const removeFile = (fileId: string) => {
    setFiles(prev => {
      const file = prev.find(f => f.id === fileId)
      if (file?.preview) {
        URL.revokeObjectURL(file.preview)
      }
      const updated = prev.filter(file => file.id !== fileId)
      onFilesChange(updated)
      return updated
    })
  }

  const { getRootProps, getInputProps, isDragActive, isDragReject } = useDropzone({
    onDrop,
    accept,
    maxFiles: multiple ? maxFiles : 1,
    maxSize,
    multiple,
    disabled,
  })

  const getFileIcon = (file: File) => {
    if (file.type.startsWith('image/')) return PhotoIcon
    if (file.type.startsWith('video/')) return FilmIcon
    if (file.type.startsWith('audio/')) return MusicalNoteIcon
    if (file.type.includes('zip') || file.type.includes('tar') || file.type.includes('gz')) return ArchiveBoxIcon
    return DocumentIcon
  }

  const getStatusIcon = (status: FileWithPreview['status']) => {
    switch (status) {
      case 'success':
        return CheckCircleIcon
      case 'error':
        return ExclamationTriangleIcon
      default:
        return null
    }
  }

  const getStatusColor = (status: FileWithPreview['status']) => {
    switch (status) {
      case 'success':
        return 'text-green-600'
      case 'error':
        return 'text-red-600'
      case 'uploading':
        return 'text-blue-600'
      default:
        return 'text-gray-600'
    }
  }

  return (
    <div className={cn('w-full', className)}>
      {/* Drop Zone */}
      <div
        {...getRootProps()}
        className={cn(
          'relative border-2 border-dashed rounded-lg p-6 transition-colors cursor-pointer',
          isDragActive && !isDragReject && 'border-primary-400 bg-primary-50 dark:bg-primary-900/20',
          isDragReject && 'border-red-400 bg-red-50 dark:bg-red-900/20',
          !isDragActive && 'border-gray-300 dark:border-gray-600 hover:border-gray-400 dark:hover:border-gray-500',
          disabled && 'opacity-50 cursor-not-allowed'
        )}
      >
        <input {...getInputProps()} />

        {children ? (
          children
        ) : (
          <div className="text-center">
            <CloudArrowUpIcon className="mx-auto h-12 w-12 text-gray-400" />
            <div className="mt-4">
              <p className="text-sm text-gray-600 dark:text-gray-400">
                {isDragActive ? (
                  isDragReject ? (
                    'Some files are not supported'
                  ) : (
                    'Drop files here'
                  )
                ) : (
                  <>
                    <span className="font-medium text-primary-600 dark:text-primary-400">
                      Click to upload
                    </span>{' '}
                    or drag and drop
                  </>
                )}
              </p>
              <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                {accept ? Object.keys(accept).join(', ') : 'Any file type'} up to {formatBytes(maxSize)}
              </p>
            </div>
          </div>
        )}
      </div>

      {/* File List */}
      {files.length > 0 && (
        <div className="mt-4 space-y-2">
          <h4 className="text-sm font-medium text-gray-900 dark:text-white">
            Uploaded Files ({files.length})
          </h4>
          <div className="space-y-2">
            {files.map((file) => {
              const FileIcon = getFileIcon(file)
              const StatusIcon = getStatusIcon(file.status)

              return (
                <div
                  key={file.id}
                  className="flex items-center space-x-3 p-3 bg-gray-50 dark:bg-gray-700 rounded-lg"
                >
                  {/* File Preview/Icon */}
                  <div className="flex-shrink-0">
                    {file.preview ? (
                      <img
                        src={file.preview}
                        alt={file.name}
                        className="h-10 w-10 object-cover rounded"
                      />
                    ) : (
                      <div className="h-10 w-10 flex items-center justify-center bg-gray-200 dark:bg-gray-600 rounded">
                        <FileIcon className="h-6 w-6 text-gray-500 dark:text-gray-400" />
                      </div>
                    )}
                  </div>

                  {/* File Info */}
                  <div className="flex-1 min-w-0">
                    <p className="text-sm font-medium text-gray-900 dark:text-white truncate">
                      {file.name}
                    </p>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      {formatBytes(file.size)}
                    </p>

                    {/* Progress Bar */}
                    {file.status === 'uploading' && typeof file.progress === 'number' && (
                      <div className="mt-1">
                        <div className="w-full bg-gray-200 dark:bg-gray-600 rounded-full h-1">
                          <div
                            className="bg-primary-600 h-1 rounded-full transition-all duration-300"
                            style={{ width: `${file.progress}%` }}
                          />
                        </div>
                      </div>
                    )}

                    {/* Error Message */}
                    {file.status === 'error' && file.error && (
                      <p className="text-xs text-red-600 dark:text-red-400 mt-1">
                        {file.error}
                      </p>
                    )}
                  </div>

                  {/* Status & Actions */}
                  <div className="flex items-center space-x-2">
                    {StatusIcon && (
                      <StatusIcon className={cn('h-5 w-5', getStatusColor(file.status))} />
                    )}

                    {file.status === 'uploading' && (
                      <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-primary-600" />
                    )}

                    <button
                      onClick={() => removeFile(file.id)}
                      className="p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 rounded-full hover:bg-gray-200 dark:hover:bg-gray-600"
                    >
                      <XMarkIcon className="h-4 w-4" />
                    </button>
                  </div>
                </div>
              )
            })}
          </div>
        </div>
      )}
    </div>
  )
}

export interface ImageUploadProps {
  onImageChange: (file: File | null) => void
  currentImage?: string
  className?: string
  size?: 'sm' | 'md' | 'lg'
  shape?: 'square' | 'circle'
  disabled?: boolean
}

export const ImageUpload: React.FC<ImageUploadProps> = ({
  onImageChange,
  currentImage,
  className,
  size = 'md',
  shape = 'square',
  disabled = false,
}) => {
  const [preview, setPreview] = useState<string | null>(currentImage || null)

  const sizeClasses = {
    sm: 'w-16 h-16',
    md: 'w-24 h-24',
    lg: 'w-32 h-32',
  }

  const onDrop = useCallback((acceptedFiles: File[]) => {
    const file = acceptedFiles[0]
    if (file) {
      const previewUrl = URL.createObjectURL(file)
      setPreview(previewUrl)
      onImageChange(file)
    }
  }, [onImageChange])

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    accept: {
      'image/*': ['.png', '.jpg', '.jpeg', '.gif', '.webp']
    },
    maxFiles: 1,
    disabled,
  })

  const removeImage = () => {
    setPreview(null)
    onImageChange(null)
  }

  return (
    <div className={cn('relative', className)}>
      <div
        {...getRootProps()}
        className={cn(
          'relative border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg overflow-hidden transition-colors cursor-pointer hover:border-gray-400 dark:hover:border-gray-500',
          shape === 'circle' && 'rounded-full',
          sizeClasses[size],
          isDragActive && 'border-primary-400 bg-primary-50 dark:bg-primary-900/20',
          disabled && 'opacity-50 cursor-not-allowed'
        )}
      >
        <input {...getInputProps()} />

        {preview ? (
          <img
            src={preview}
            alt="Upload preview"
            className="w-full h-full object-cover"
          />
        ) : (
          <div className="w-full h-full flex items-center justify-center bg-gray-50 dark:bg-gray-700">
            <PhotoIcon className="h-8 w-8 text-gray-400" />
          </div>
        )}

        {/* Overlay */}
        <div className="absolute inset-0 bg-black bg-opacity-0 hover:bg-opacity-20 transition-opacity flex items-center justify-center">
          <CloudArrowUpIcon className="h-6 w-6 text-white opacity-0 hover:opacity-100 transition-opacity" />
        </div>
      </div>

      {/* Remove button */}
      {preview && (
        <button
          onClick={(e) => {
            e.stopPropagation()
            removeImage()
          }}
          className="absolute -top-1 -right-1 p-1 bg-red-500 text-white rounded-full hover:bg-red-600 transition-colors"
        >
          <XMarkIcon className="h-3 w-3" />
        </button>
      )}
    </div>
  )
}